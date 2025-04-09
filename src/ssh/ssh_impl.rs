use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, Channel};
use super::ssh_struct::ssh;
use std::io::Read;
use std::{fs, io};

use walkdir::WalkDir;

use log::error;


impl ssh {
    pub fn new() -> ssh {
        ssh { session: Session::new().unwrap()  } 
    }

    /// Connect to ssh server
    ///
    /// # Example 
    ///
    /// ```
    /// use DataOrchestra::ssh::ssh_struct::ssh;
    /// let ssh = ssh::new();
    /// ```
    pub fn connect(&mut self, host: &String, port: u16, username: &String, password: &String) {
        let tcp = TcpStream::connect(format!("{host}:{port}")).expect("Unable to setup tcp stream");
        self.session.set_tcp_stream(tcp);
        
        let handshake: Result<(), ssh2::Error> = self.session.handshake();

        if let Err(ref error) = handshake {
            error!("Unsuccessful handshake {}", error);
        }
        
        let authentication: Result<(), ssh2::Error> = self.session.userauth_password(username, password);

        if let Err(ref error) = authentication {
            error!("Unsuccessful authentication {}", error);
        }

        assert!(self.session.authenticated()); 
    }


    /// Execute command over ssh connection
    ///
    /// # Example
    ///
    /// ```
    /// use DataOrchestra::ssh::ssh_struct::ssh;
    /// let ssh = ssh::new();
    /// let result = ssh.exec("pwd");
    /// println!("{}", result);
    ///
    /// ```
    pub fn exec(&self, command: &str) -> String {
        let channel: Result<Channel, ssh2::Error> = self.session.channel_session();

        if let Err(ref error) = channel {
            error!("Unable to open ssh channel {}", error);
        }

        let mut channel = channel.unwrap();
        let exec: Result<(), ssh2::Error> = channel.exec(command);
        
        if let Err(ref error) = exec {
            error!("Unable to execute command {}", error);
        }

        let mut result = String::new();
        let read: Result<usize, io::Error> = channel.read_to_string(&mut result);

        if let Err(ref error) = read {
            error!("Unable to read command output {}", error);
        }

        let close: Result<(), ssh2::Error> = channel.wait_close();
        if let Err(ref error) = close {
            error!("Unable to close channel {}", error);
        }

        result
    }


    pub fn upload_file(&self, file: &Path, location: &Path) -> Result<(), ssh2::Error>{
        let mut local_file = File::open(file).unwrap();
        let remote_file: Result<Channel, ssh2::Error> = self.session.scp_send(location, 0o644, fs::metadata(file).unwrap().len(), None);

        if let Err(ref error) = remote_file {
            error!("Unable to upload file {}", error);
        }

        let mut remote_file = remote_file.unwrap();
        
        let mut buffer = Vec::new();
        local_file.read_to_end(&mut buffer);
        remote_file.write_all(&buffer).unwrap();

        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();  
        return Ok(());
    }

    pub fn upload_directory(&self, dir: &Path, location: &Path) -> Result<(), ssh2::Error> {
        for entry in WalkDir::new(dir) {
            if let Ok(ref entry) = entry {
                let remote_path: String = format!("{}/{}", location.display(), entry.path().display()); 
                if entry.file_type().is_dir() {
                    self.exec(remote_path.as_str());
                }
                else {
                    self.upload_file(entry.path(), remote_path);
                }
            }
            println!("{}", entry.unwrap().path().display());
        }

        Ok(())
    }
}

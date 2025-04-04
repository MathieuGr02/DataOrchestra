use std::env::args;
use std::process::{Command, Child, Stdio};
use log::{debug, error, info, warn};
use crate::docker::docker_struct::Docker;
use crate::command::command_func::{output_command, spawn_command, spawn_commands};
use crate::remote::remote_trait::Remote;

impl Docker {
    /// Create docker container based on specified data
    pub fn init(&mut self) -> bool {
        info!("Initializing docker container");

        // Create network
        let networks: String = output_command("docker network ls");
        if !networks.contains(&self.network) {
            info!("{}", format!("Creating network bridge {}", &self.network));
            let create_bridge = spawn_command(&format!("docker network create -d bridge {}", &self.network)).wait();

            if create_bridge.is_err() || (create_bridge.is_ok() && !&create_bridge.as_ref().unwrap().success()) {
                warn!("Unable to create bridge {} | Code : {}", &self.network, &create_bridge.unwrap().code().unwrap());
            }
            else {
                info!("Successfully created bridge \"{}\"", &self.network)
            }
        }

        // Create image
        let containers: String = output_command("docker ps");
        if containers.contains(&self.name){
            warn!("Re initiating container {}", &self.name);
            let container_stop = spawn_command(&format!("docker stop {}", &self.name)).wait();
            let container_rm = spawn_command(&format!("docker rm {}", &self.name)).wait();

        }
        let docker = spawn_command(&format!("docker run {}", self.get_options())).wait();
        if docker.is_err() || (docker.is_ok() && !&docker.as_ref().unwrap().success()) {
            warn!("Unable to create docker container \"{}\" | Code : {}", &self.name, &docker.unwrap().code().unwrap());
        }
        else {
            info!("Successfully created docker container \"{}\"", &self.name)
        }

        // get ip
        let result = Command::new("sh")
            .arg("-c")
            .arg("docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' store")
            .output();

        match result {
            Ok(value) => info!("Docker container \"{}\" ip : {:?}", &self.name, String::from_utf8(value.stdout).unwrap()),
            Err(value) => warn!("Unable to retrieve ip address from {} | Code : {}", &self.name, value),
        };
        
        // Install ssh server
        info!("Installing shh server on {}", &self.name);
        self.execute("apt-get update").wait();
        self.execute("apt-get install -y openssh-server").wait();
        self.execute("mkdir /var/run/sshd").wait();
        self.execute("echo \"root:password\" | chpasswd").wait();
        self.execute("echo \"PermitRootLogin yes\" >> /etc/ssh/sshd_config").wait();
        self.execute("/usr/sbin/sshd -D");
        
        true
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        command = format!("{command} --network={}", &self.network);

        command = format!("{command} --name={}", &self.name);

        //command = format!("{command} -p {}:{}", &self.address.port, &self.address.internal_port);

        if let Some(options) = &self.options {
            for (key, value) in options {
                command = format!("{command} -e {key}={value}")
            }
        }

        if let Some(value) = &self.image {
            command = format!("{command} -it {value}");
        }

        if let Some(source) = &self.mount {
            if let Some(target) = &self.target {
                command = format!("{command} --mount source={source}, target={target}");
            }
        }

        // Install ssh on docker 
        // apt-get update
        // apt-get install openssh-server
        // mkdir /var/run/sshd
        // echo "root:password" | chpasswd
        // echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
        // /usr/sbin/sshd -D
        //
        // ssh root@localhost -p [port]
        // password: password

        command
    }
}

impl Remote for Docker {
    fn connect(&self) {
    
    }

    /// Execute command remotely in docker container
    ///
    /// # Examples
    /// 
    /// ```
    /// use DataOrchestra::command::Docker;
    /// let docker: Docker = Docker { image: "ubuntu" };
    /// docker.execute(&["pwd"])
    /// ```
    fn execute(&self, arg: &str) -> Child {
        debug!("{}", format!("Running command: docker exec -it {} {}", &self.name, arg));
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg(format!("/C docker exec -it {} {}", &self.name, arg))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("docker exec -it {} {}", &self.name, arg))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        };

        output    
    }

    fn get_port(&self) {
        
    }

    fn get_ip(&self) {
        
    }

    fn get_host(&self) {
        
    }
}

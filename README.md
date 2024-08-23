

# Sphinx Auth Server

Sphinx is an auth server written in rust.

- **Objective**:
This project aims to me be a simple auth server that covers *almost* all 
your auth needs when developing new projects.

# Roadmap
## Core Functionality
- [X] Email/password login
- [X] Verify email
- [X] Reset-password 
- [ ] Oauth login
- [ ] OTPs
- [ ] Magic links
- [ ] Multi factor authentication

## Deployment Options
- [X] Docker Compose
- [ ] Kubernetes
- [ ] Terraform for various cloud providers

# Architecture
- This is how the server is aimed to work in relation to other services.

![sphinx auth server architecture](https://i.ibb.co/7r8rHSQ/image.png)


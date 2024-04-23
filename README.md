# California Pay Check Estimator

This Rust-based application provides estimates of monthly and annual net salaries after tax deductions based on the
user's gross yearly income. It uses major tax-related data to calculate the net pay.

## Project Limitations

Currently, the application does not account for:

- Contributions to retirement plans such as 401(k)s,
- Health insurance premiums,
- Other deductions such as financial obligations,
- Tax credits or deductions for dependents.

## Continuous Integration

The project is integrated with multiple CI/CD systems:

- GitLab CI: Automated testing and builds within the GitLab ecosystem.
- GitHub Actions: Provides support for workflows that automatically build and test the code on GitHub.
- Local Julia CI Script: A custom script that simulates major CI/CD environments for local testing and debugging.

### Local Julia CI Script

The local CI script developed in Julia performs the following tasks:

- Builds a Docker container.
- Installs the Clippy tool for Rust linting, as it's not included in the default container setup.
- Runs Docker Scout to analyze the container for security vulnerabilities and other issues.
- Executes Clippy for code linting.
- Runs unit tests within the container environment.

#### Prerequisites

- Docker must be installed without requiring sudo privileges and may require Docker Desktop running in the background, 
depending on your Docker setup.
- Julia needs to be installed on your system, which you can do from the Julia language website. The installation 
is managed via a curl pipe script similar to rustup, using juliaup.

#### Running the Script

To run the local CI/CD script, execute the following command from the root directory of the project, 
where the Dockerfile is located:

```julia
julia .script/run_dev_ci_cd.jl
```

Please note that the script needs access to the project's Dockerfile to function correctly.
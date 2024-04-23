using Printf

# Function to build the Docker image
function buildDockerImage(imageName::String)
    println("🏗️ Building Docker image...")
    try
        run(`docker build -t $imageName .`)
        run(`docker scout cves local://$imageName:latest`)
        println("✅ Docker image built successfully!")
    catch e
        @error("❌ Failed to build Docker image", exception=e)
        exit(1)
    end
end

# Function to run Docker commands and handle specific errors
function runCommand(label::String, cmd::Cmd)
    println("👟 Running $label...")
    try
        run(cmd)
        println("✅ $label passed successfully.")
    catch e
        @error("❌ Command execution failed", label, exception=e)
        exit(1)
    end
end

function runCleanup(imageName::String)
    println("🫧 Cleaning up ....")
    try
        run(`docker rmi $imageName`)
        println("🧼 Docker image $imageName removed successfully.")
    catch e
        @error("❌ Failed to remove image", imageName, exception=e)
    end
end

function main()
    imageName::String = "paycheck_estimator"
    buildDockerImage(imageName)
    runCommand("Clippy Lint", `docker run --rm $imageName cargo clippy -- -D warnings`)
    runCommand("Unit Tests", `docker run --rm $imageName cargo test`)
    runCleanup(imageName)
    println("✅ All integration tests completed successfully!")
end

main()
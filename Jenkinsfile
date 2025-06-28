pipeline {
    agent any

    stages {
        stage('Clone Repository') {
            steps {
                git url: 'https://github.com/MohAli2106/Multithread_data_logger '
            }
        }

        stage('Check Files') {
            steps {
                sh 'ls -la'  // Make sure Cargo.toml, docfile, and src exist
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -f docfile -t data_logger .'
            }
        }

        stage('Run Docker Container') {
            steps {
                sh 'docker run -d -p 7877:7877 --name my_logger data_logger'
            }
        }

        stage('View Logs') {
            steps {
                sh 'docker logs my_logger'
            }
        }

        stage('Keep Container Running for Testing') {
            steps {
                echo 'Container is running. You can now access the dashboard at http://<your-jenkins-machine-ip>:7877/'
                input message: 'Press OK to stop the container', submitter: 'Mohamed Moustafa'
            }
        }

        stage('Stop and Remove Container') {
            steps {
                sh 'docker stop my_logger || true'
                sh 'docker rm my_logger || true'
            }
        }
    }

    post {
        success {
            echo 'Build succeeded! You successfully built and tested a Dockerized Rust app using Jenkins.'
        }
        failure {
            echo 'Build failed. Check the logs above to debug.'
            emailext (
                subject: "FAILED: Job '${env.JOB_NAME} [${env.BUILD_NUMBER}]'",
                body: '''The Jenkins pipeline failed.

Check the logs for details:
${BUILD_URL}''',
                recipientProviders: [[$class: 'DevelopersRecipientProvider']]
            )
        }
    }
}







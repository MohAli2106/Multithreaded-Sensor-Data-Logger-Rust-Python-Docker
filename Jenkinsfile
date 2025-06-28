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
                sh 'ls -la'
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -f docfile -t data_logger .'
            }
        }

        stage('Stop Existing Container (If Any)') {
            steps {
                echo 'Stopping and removing any existing container...'
                sh '''
                    docker stop my_logger || true
                    docker rm my_logger || true
                '''
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

        stage('Archive Sensor Log') {
            steps {
                sh 'mkdir -p logs && docker cp my_logger:/app/sensor_data.log logs/'
                archiveArtifacts artifacts: 'logs/sensor_data.log'
            }
        }
    }

    post {
        success {
            echo 'Build succeeded! Rust app ran successfully and sensor data was logged.'
        }
        failure {
            echo 'Build failed. Check the logs above to debug.'
        }
    }
}

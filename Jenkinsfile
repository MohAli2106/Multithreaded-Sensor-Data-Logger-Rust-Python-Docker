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

        stage('Run Docker Container') {
            steps {
                sh 'docker run --name sensor_logger data_logger'
            }
        }

        stage('Archive Sensor Log') {
            steps {
                echo 'Saving sensor log file...'
                sh '''
                    mkdir -p logs
                    docker cp my_logger:/app/sensor_data.log logs/
                '''
                archiveArtifacts artifacts: 'logs/sensor_data.log'
            }
        }
    }

    post {
        success {
            echo '✅ Build succeeded! Your Rust-based data logger ran successfully and logged sensor data.'
        }
        failure {
            echo '❌ Build failed. Check the logs above to debug.'
        }
    }
}

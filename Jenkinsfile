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
         stage('Stop Existing Container') {
            steps {
                echo 'Removing old container (if exists)...'
                sh '''
                     docker stop logger || true
                     docker rm logger || true
                 '''
    }
}
        stage('Run Docker Container') {
            steps {
                sh 'docker run --name loger data_logger'
            }
        }

        stage('Archive Sensor Log') {
            steps {
                echo 'Saving sensor log file...'

            }
        }
        stage('Clean Up') {
            steps {
                sh 'docker rm -f sensor_logger || true'
                sh 'docker rmi data_logger || true'
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

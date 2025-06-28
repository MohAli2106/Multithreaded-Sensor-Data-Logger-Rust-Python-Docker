





pipeline {
    agent any

    stages {
        stage('Clone Repository') {
            steps {
                git url: 'https://github.com/MohAli2106/Multithread_data_logger '
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -f docfile-t data_logger .'
            }
        }

        stage('Run Docker Container') {
            steps {
                sh 'docker run -d -p 7877:7877 --name my_logger data_logger'
            }
        }

        stage('Check Container Logs') {
            steps {
                sh 'docker logs my_logger'
            }
        }

        stage('Stop and Remove Container') {
            steps {
                sh 'docker stop my_logger || true'
                sh 'docker rm my_logger || true'
            }
        }
    }
}













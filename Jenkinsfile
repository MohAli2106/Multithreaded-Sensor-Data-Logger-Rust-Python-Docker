pipeline{
agent any
stages{
    stage('clone repo'){
        steps{
            git url: 'https://github.com/MohAli2106/Multithread_data_logger '
        }
    }
    stage('build the app'){
        steps {
            sh 'cargo build --releease'
            }
            
    }

    stage('run the app'){
        steps {
            sh 'cargo run --release'
            }
    }
    stage('Build Docker Image (Optional)') {
            steps {
                sh 'docker build -t data-logger .'
            }
        }

        stage('Run App (Optional)') {
            steps {
                sh 'docker run -d -p 7877:7877 data-logger'
            }
        }
    
}






















}

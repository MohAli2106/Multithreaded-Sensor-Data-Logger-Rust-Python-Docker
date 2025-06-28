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
    
}






















}
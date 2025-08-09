pipeline {
    agent any

    environment {
        DOCKER_IMAGE_NAME = 'sensor-dashboard'
        CONTAINER_NAME = 'sensor-monitor'
    }

    stages {
        stage('Clone Repository') {
            steps {
                git branch: 'main', url: 'https://github.com/MohAli2106/Multithreaded-Sensor-Data-Logger-Rust-Python-Docker.git'
            }
        }

        stage('Check Files') {
            steps {
                sh 'ls -la'
            }
        }

        stage('Build Docker Image') {
            steps {
                sh '''
                    echo "Building Docker image..."
                    docker build --no-cache -t ${DOCKER_IMAGE_NAME} .
                '''
            }
        }

        stage('Stop Existing Container') {
            steps {
                echo 'Stopping existing container (if any)...'
                sh '''
                    docker stop ${CONTAINER_NAME} || true
                    docker rm ${CONTAINER_NAME} || true
                '''
            }
        }

        stage('Run Docker Container') {
            steps {
                script {
                    echo 'Starting container and waiting for data...'
                    sh '''
                        # Run container in background
                        docker run -d --name ${CONTAINER_NAME} ${DOCKER_IMAGE_NAME}
                        
                        # Wait 10 seconds for data to be collected
                        sleep 10
                        
                        # Check if log file exists and has content
                        if [ -f sensor_data.log ] && [ -s sensor_data.log ]; then
                            echo "✅ Log file created and populated"
                        else
                            echo "❌ Log file missing or empty"
                            docker logs ${CONTAINER_NAME}
                            exit 1
                        fi
                        
                        # Get container logs for debugging
                        docker logs ${CONTAINER_NAME}
                    '''
                }
            }
        }

        stage('Verify Container Status') {
            steps {
                script {
                    def containerStatus = sh(script: "docker ps -f name=${CONTAINER_NAME} --format '{{.Status}}'", returnStdout: true).trim()
                    if (containerStatus.contains("Up")) {
                        echo "✅ Container is running"
                    } else {
                        echo "❌ Container not running: ${containerStatus}"
                        sh "docker logs ${CONTAINER_NAME}"
                        error "Container failed to start properly"
                    }
                }
            }
        }

        stage('Extract Artifacts') {
            steps {
                script {
                    // Copy log file from container
                    sh '''
                        docker cp ${CONTAINER_NAME}:/workspace/sensor_data.log ./sensor_data.log
                        docker cp ${CONTAINER_NAME}:/workspace/python/static/plot.png ./plot.png
                    '''
                    
                    // Verify artifacts exist
                    sh '''
                        if [ -f sensor_data.log ]; then
                            echo "✅ sensor_data.log extracted"
                            head -n 5 sensor_data.log
                        else
                            echo "❌ sensor_data.log not found"
                        fi
                        
                        if [ -f plot.png ]; then
                            echo "✅ plot.png extracted"
                        else
                            echo "❌ plot.png not found"
                        fi
                    '''
                }
            }
        }

        stage('Archive Artifacts') {
            steps {
                archiveArtifacts artifacts: 'sensor_data.log, plot.png', allowEmptyArchive: true
                stash includes: 'sensor_data.log, plot.png', name: 'output-files'
            }
        }

        stage('Clean Up') {
            steps {
                sh '''
                    echo "Cleaning up..."
                    docker stop ${CONTAINER_NAME} || true
                    docker rm ${CONTAINER_NAME} || true
                    docker rmi ${DOCKER_IMAGE_NAME} || true
                '''
            }
        }
    }

    post {
        success {
            echo '✅ Jenkins pipeline completed successfully!'
            echo '📎 Artifacts (sensor_data.log, plot.png) are available in the build'
            mail to: 'team@example.com', subject: 'Pipeline Success', body: 'Sensor monitoring system built and tested successfully!'
        }
        failure {
            echo '❌ Jenkins pipeline failed!'
            mail to: 'admin@example.com', subject: 'Pipeline Failed', body: "Build failed: ${env.BUILD_URL}"
        }
    }
}

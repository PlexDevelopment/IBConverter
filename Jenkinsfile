pipeline {
  agent {
    docker { image 'rust:latest' }
  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build --release'
      }
      post {
        always {
		  archiveArtifacts artifacts: 'target/release/*.exe', followSymlinks: false
        }
      }
    }
  }
}
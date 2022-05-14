pipeline {
  agent any
  stages {
    stage('Build') {
      withDockerContainer('rust:latest') {
        steps {
          sh 'cargo build --release'
        }
      }
      post {
        always {
		  archiveArtifacts artifacts: 'target/release/*.exe', followSymlinks: false
        }
      }
    }
  }
}
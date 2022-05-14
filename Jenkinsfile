pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
				withDockerContainer('rust:latest') {
					sh 'cargo build --release'
				}
			}
		}
	}
    post {
      always {
		archiveArtifacts artifacts: 'target/release/*.exe', followSymlinks: false
		}
	}
}
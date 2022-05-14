pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                withDockerContainer(image: 'rust:latest', toolName: 'Docker') {
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
pipeline {
  agent {
    kubernetes {
      yamlFile 'KubernetesBuilder.yaml'
    }
  }
  stages {
    stage('Build') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo build --release'
        }
      }
    }
    stage('Test') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo test'
        }
      }
    }
    stage('Copy Artifacts') {
      steps {
        container('rust') {
          sh 'cp -r * /workspace/opt/app/shared/'
        }
      }
    }
    stage('Release') {
      steps {
        container('kaniko') {
          sh 'cp -r /workspace/opt/app/shared/*  /workspace'
          sh 'ulimit -n 10000'
          sh '/kaniko/executor -f Dockerfile --destination=docker.ultimaengineering.io/trade_forge_api:${BRANCH_NAME}-${BUILD_NUMBER} --destination=docker.ultimaengineering.io/trade_forge_api:latest'
        }
      }
    }
  }
}

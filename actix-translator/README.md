# Mini Project week4: Actix containerized calculator
Deployed on: https://eeixxfhm34.us-east-1.awsapprunner.com/translate

## Containerization
1. In [lib.rs](https://github.com/nogibjj/Yuzhou-IDS721-Proj2/blob/main/actix-translator/src/lib.rs), implement calculator logics 
2. In [main.rs](https://github.com/nogibjj/Yuzhou-IDS721-Proj2/blob/main/actix-translator/src/main.rs), implement APIs with actix_web
3. In [Dockerfile](https://github.com/nogibjj/Yuzhou-IDS721-Proj2/blob/main/actix-translator/Dockerfile), configure containerization
4. Build docker with `docker build -t rust-bert-rocket .`
5. Test docker containerized app with `docker run -p 8080:8080 rust-bert-rocket`

## Deployment
1. In AWS Elastic Container Registry (ECR), create new container repository `actix-translator`
2. Build docker for our own app: `make build`
3. Check docker can run: `make rundocker`
4. Follow push commands inside newly created ECR repo, push our local docker image to `actix-translator`

    i. Configure identification and authentication by running
        ```
        aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin {as shown in push commands}
        ```
        inside AWS Cloud9
        
    ii. Build image of actix-translator
        ```
        docker build -t actix-translator .
        ```
        
    iii. Tag local repo image `actix-translator` with remote repo image
        ```
        docker tag actix:latest 773627151292.dkr.ecr.us-east-1.amazonaws.com/actix:latest
        ```
        
    iv. Push your image to ECR repo
        ```
        docker push 773627151292.dkr.ecr.us-east-1.amazonaws.com/actix:latest
        ```
        
3. Deploy the containerized app in AWS App Runner

    i. Click `Create Service`
    
    ii. Configure source with Amazon ECR, browse and choose image URI as the one you just pushed (`actix-translator` here)
    
    iii. Create our use role `AppRunnerECRAccessRole`
    
    iv. Create and Deploy, done!!!
    
    



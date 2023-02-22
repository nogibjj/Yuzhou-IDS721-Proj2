# Mini Project week4: Actix containerized calculator
Deployed on: https://eeixxfhm34.us-east-1.awsapprunner.com/

Usage: https://eeixxfhm34.us-east-1.awsapprunner.com/calculate/1+(2*3)

## Containerization
1. In [lib.rs](https://github.com/nogibjj/Yuzhou-rust-mini-projects/blob/main/week4-actix/src/lib.rs), implement calculator logics 
2. In [main.rs](https://github.com/nogibjj/Yuzhou-rust-mini-projects/blob/main/week4-actix/src/main.rs), implement APIs with actix_web
3. In [Dockerfile](https://github.com/nogibjj/Yuzhou-rust-mini-projects/blob/main/week4-actix/Dockerfile), configure containerization
4. Build docker with ` docker build -t calculator .`
5. Test docker containerized app with `docker run -p 8080:8080 calculator`

## Deployment
1. In AWS Elastic Container Registry (ECR), create new container repository `actix`
2. Follow push commands inside newly created ECR repo, push our local docker image to `actix`

    i. Configure identification and authentication by running
        ```
        aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin {as shown in push commands}
        ```
        inside AWS Cloud9
        
    ii. Build image of APP
        ```
        docker build -t actix .
        ```
        
    iii. Tag local repo image `actix` with remote repo image `773627151292.dkr.ecr.us-east-1.amazonaws.com/actix:latest`
        ```
        docker tag actix:latest 773627151292.dkr.ecr.us-east-1.amazonaws.com/actix:latest
        ```
        
    iv. Push your image to ECR repo
        ```
        docker push 773627151292.dkr.ecr.us-east-1.amazonaws.com/actix:latest
        ```
        
3. Deploy the containerized app in AWS App Runner

    i. Click `Create Service`
    
    ii. Configure source with Amazon ECR, browse and choose image URI as the one you just pushed (`actix` here)
    
    iii. Create our use role `AppRunnerECRAccessRole`
    
    iv. Create and Deploy, done!!!
    
    



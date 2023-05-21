import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class RustCdkLambdaStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'RustCdkLambdaQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });


    new RustFunction(this, 'RustFunction', {
      directory: 'lambdas',
      bin: "hello_world"
    })
  }
}

import * as cdk from 'aws-cdk-lib';
import { BillingMode, Table } from 'aws-cdk-lib/aws-dynamodb';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class RustCdkLambdaStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);


    // Create a dynamo table to track invocations.
    const db = new Table(this, 'RustCdkLambdaTable', {
      partitionKey: { name: 'id', type: cdk.aws_dynamodb.AttributeType.STRING },
      removalPolicy: cdk.RemovalPolicy.DESTROY, // Don't care if this gets deleted.
      billingMode: BillingMode.PAY_PER_REQUEST,

    });

    const fn = new RustFunction(this, 'RustFunction', {
      directory: 'lambdas',
      bin: "hello_world",
      environment: {
        "TABLE_NAME": db.tableName,
      }
    })

    db.grantReadWriteData(fn);
  }
}

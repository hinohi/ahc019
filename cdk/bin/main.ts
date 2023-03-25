import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as sfn from "aws-cdk-lib/aws-stepfunctions";
import * as tasks from "aws-cdk-lib/aws-stepfunctions-tasks";
import * as logs from "aws-cdk-lib/aws-logs";
import * as path from "path";

export class Ahc019 extends cdk.Stack {
  constructor(scope: cdk.App, id: string) {
    super(scope, id);

    const lambdaFunction = new lambda.DockerImageFunction(this, "Function", {
      functionName: "ahc019",
      memorySize: 1792,
      timeout: cdk.Duration.seconds(20),
      code: lambda.DockerImageCode.fromImageAsset(
        path.join(__dirname, "../.."),
        {
          file: "Dockerfile",
        }
      ),
    });

    const log = new logs.LogGroup(this, 'LogGroup', {
      logGroupName: 'ahc019',
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const task = new tasks.LambdaInvoke(this, "Invoke", {
      lambdaFunction,
      outputPath: '$.Payload',
    });
    const map = new sfn.Map(this, "Map");
    map.iterator(task);
    new sfn.StateMachine(this, "StateMachine", {
      stateMachineName: "AHC019",
      stateMachineType: sfn.StateMachineType.STANDARD,
      definition: map,
      timeout: cdk.Duration.hours(1),
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      tracingEnabled: false,
      logs: {
        destination: log,
        level: sfn.LogLevel.ERROR,
        includeExecutionData: true,
      },
    });
  }
}

const app = new cdk.App();
new Ahc019(app, "ahc019");

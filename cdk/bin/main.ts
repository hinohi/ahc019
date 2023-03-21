import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as path from "path";

export class Ahc019 extends cdk.Stack {
  constructor(scope: cdk.App, id: string) {
    super(scope, id);

    new lambda.DockerImageFunction(this, "Function", {
      functionName: "ahc019",
      memorySize: 1792,
      timeout: cdk.Duration.seconds(10),
      code: lambda.DockerImageCode.fromImageAsset(
        path.join(__dirname, "../.."),
        {
          file: "Dockerfile",
        }
      ),
    });
  }
}

const app = new cdk.App();
new Ahc019(app, "ahc019");

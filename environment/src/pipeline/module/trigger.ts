import * as gcp from '@pulumi/gcp';
import * as pulumi from '@pulumi/pulumi';

import {
  Region,
  ProductName,
  PipelineProject,
  RegistryName,
  Environment,
  PRODUCTION_PROJECT,
  STAGING_PROJECT,
  PRODUCT_NAME,
  RUN_NAME,
} from '../../constant';
import { EnvService } from '../../lib/env.service';

type BuildStep = pulumi.Input<gcp.types.input.cloudbuild.TriggerBuildStep>;

type ContainerTag = {
  test: string;
  production: string;
  latest: string;
};

type StepName = {
  kaniko: string;
  docker: string;
  gcloud: string;
};

type Props = {
  region: Region;
  pipelineProject: PipelineProject;
  productName: ProductName;
  registryName: RegistryName;
};

export class Trigger {
  readonly region: Region;
  readonly projectName: PipelineProject;
  readonly productName: ProductName;
  readonly registryName: RegistryName;

  readonly testImage: string;
  readonly productionImage: string;

  readonly containerTag: ContainerTag;
  readonly stepName: StepName;

  readonly triggerName: string;

  constructor(private envService: EnvService, props: Props) {
    this.region = props.region;
    this.projectName = props.pipelineProject;
    this.productName = props.productName;
    this.registryName = props.registryName;

    this.triggerName = `${props.productName}-pipeline-trigger`;

    const prefix = [
      `${this.region}-docker.pkg.dev`,
      `${this.projectName}`,
      `${this.registryName}`,
    ].join('/');
    this.testImage = `${prefix}/${props.productName}-test-image`;
    this.productionImage = `${prefix}/${props.productName}-production-image`;

    this.containerTag = {
      test: `${this.testImage}:$COMMIT_SHA`,
      production: `${this.productionImage}:$COMMIT_SHA`,
      latest: `${this.productionImage}:latest`,
    };

    this.stepName = {
      kaniko: 'gcr.io/kaniko-project/executor:latest',
      docker: 'gcr.io/cloud-builders/docker',
      gcloud: 'gcr.io/google.com/cloudsdktool/cloud-sdk',
    };
  }

  get image() {
    return this.productionImage;
  }

  create(): gcp.cloudbuild.Trigger {
    return new gcp.cloudbuild.Trigger(this.triggerName, {
      name: this.triggerName,
      description: 'The pipeline for modggl',
      tags: [PRODUCT_NAME],
      github: {
        owner: this.envService.githubOwner.value,
        name: this.envService.githubRepository.value,
        push: {
          branch: this.envService.githubBranch.value,
        },
      },
      build: {
        timeout: '7200s',
        steps: [
          this.buildForTest,
          this.test,
          this.build,
          this.deployStaging,
          this.deployProduction,
        ],
      },
    });
  }

  private get buildForTest(): BuildStep {
    return {
      id: 'Build for Test',
      name: this.stepName.kaniko,
      args: [
        `--context=dir://${this.productName}`,
        `--target=build`,
        `--destination=${this.containerTag.test}`,
        `--cache=true`,
      ],
    };
  }

  private get test(): BuildStep {
    return {
      id: 'Test',
      name: this.stepName.docker,
      dir: this.productName,
      entrypoint: 'bash',
      args: [
        '-c',
        `docker run ${this.containerTag.test}`,
        'cargo test',
        '--all',
        '--release',
        '--target x86_64-unknown-linux-musl',
      ],
    };
  }

  private get build(): BuildStep {
    return {
      id: 'Build',
      name: this.stepName.kaniko,
      args: [
        `--context=dir://${this.productName}`,
        `--destination=${this.containerTag.latest}`,
        `--destination=${this.containerTag.production}`,
        `--cache=true`,
      ],
    };
  }

  private get deployStaging(): BuildStep {
    return {
      id: 'Deploy staging',
      name: this.stepName.gcloud,
      entrypoint: 'gcloud',
      args: this.deployArgs(STAGING_PROJECT),
    };
  }

  private get deployProduction(): BuildStep {
    return {
      id: 'Deploy production',
      name: this.stepName.gcloud,
      entrypoint: 'gcloud',
      args: this.deployArgs(PRODUCTION_PROJECT),
    };
  }

  private deployArgs(environment: Environment) {
    return [
      'run',
      'deploy',
      RUN_NAME,
      '--platform=managed',
      `--image=${this.containerTag.latest}`,
      '--labels=commit-sha=$COMMIT_SHA,gcb-build-id=$BUILD_ID',
      `--region=${this.region}`,
      '--port=8080',
      '--quiet',
      '--allow-unauthenticated',
      `--project=${environment}`,
    ];
  }
}

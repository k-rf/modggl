import * as gcp from '@pulumi/gcp';

import {
  Region,
  ProductName,
  ProjectName,
  RegistryName,
  Environment,
  PRODUCTION,
  STAGING,
  PRODUCT_NAME,
} from '../../constant';

type DeployArgsProps = {
  region: Region;
  productName: ProductName;
  imageTag: string;
  environment: Environment;
};

const deployArgs = ({
  region,
  productName,
  imageTag,
  environment,
}: DeployArgsProps) => [
  'run',
  'deploy',
  productName,
  '--platform=managed',
  `--image=${imageTag}`,
  '--labels=commit-sha=$COMMIT_SHA,gcb-build-id=$BUILD_ID',
  `--region=${region}`,
  '--port=8080',
  '--quiet',
  '--allow-unauthenticated',
  `--project=${environment}`,
];

type Props = {
  region: Region;
  projectName: ProjectName;
  productName: ProductName;
  registryName: RegistryName;
};

export const createTrigger = ({
  region,
  projectName,
  productName,
  registryName,
}: Props) => {
  const imageName = `${productName}-image`;
  const triggerName = `${productName}-pipeline-trigger`;
  const stepName = {
    docker: 'gcr.io/cloud-builders/docker',
    gcloud: 'gcr.io/cloud-builders/gcloud-slim',
  };
  const containerTag = {
    test: `${imageName}:test`,
    run: [
      `${region}-docker.pkg.dev`,
      `${projectName}`,
      `${registryName}`,
      `${imageName}:$COMMIT_SHA`,
    ].join('/'),
  };

  return new gcp.cloudbuild.Trigger(triggerName, {
    name: triggerName,
    description: 'The pipeline for modggl',
    tags: [PRODUCT_NAME],
    github: {
      owner: 'poiulkjhmnbv',
      name: 'modggl',
      push: {
        branch: '^main$',
      },
    },
    build: {
      steps: [
        {
          id: 'Build for Test',
          name: stepName.docker,
          dir: productName,
          entrypoint: 'bash -c',
          args: [
            'docker build .',
            '--target build',
            `--tag ${containerTag.test}`,
          ],
        },
        {
          id: 'Test',
          name: stepName.docker,
          dir: productName,
          entrypoint: 'bash -c',
          args: [
            `docker run ${containerTag.test}`,
            'cargo test',
            '--all',
            '--release',
            '--target x86_64-unknown-linux-musl',
          ],
        },
        {
          id: 'Build',
          name: stepName.docker,
          dir: productName,
          entrypoint: 'bash -c',
          args: ['docker build .', `--tag ${containerTag.run}`],
        },
        {
          id: 'Push',
          name: stepName.docker,
          args: ['push', containerTag.run],
        },
        {
          id: 'Deploy staging',
          name: stepName.gcloud,
          args: deployArgs({
            region,
            productName,
            imageTag: containerTag.run,
            environment: STAGING,
          }),
        },
        {
          id: 'Deploy production',
          name: stepName.gcloud,
          args: deployArgs({
            region,
            productName,
            imageTag: containerTag.run,
            environment: PRODUCTION,
          }),
        },
      ],
    },
  });
};

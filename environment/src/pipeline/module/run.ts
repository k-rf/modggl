import * as gcp from '@pulumi/gcp';

import {
  Environment,
  PRODUCTION_PROJECT,
  REGION,
  RUN_NAME,
} from '../../constant';
import { EnvService } from '../../lib/env.service';

type Props = {
  environment: Environment;
  image: string;
};

export class Run {
  readonly environment: Environment;
  readonly image: string;

  constructor(private envService: EnvService, props: Props) {
    this.environment = props.environment;
    this.image = props.image;
  }

  create(): gcp.cloudrun.Service {
    const serviceAccountName =
      this.environment === PRODUCTION_PROJECT
        ? this.envService.serviceAccountRunProduction.value
        : this.envService.serviceAccountRunStaging.value;

    return new gcp.cloudrun.Service(`${this.environment}-run`, {
      name: RUN_NAME,
      project: this.environment,
      location: REGION,
      metadata: {
        annotations: {
          'run.googleapis.com/ingress': 'internal',
          'run.googleapis.com/ingress-status': 'internal',
        },
      },
      template: {
        spec: {
          serviceAccountName,
          containers: [
            {
              image: this.image,
              ports: [{ containerPort: 8080 }],
              envs: [
                this.envService.reportApi,
                this.envService.slackWebHookUrl,
                this.envService.togglApi,
                this.envService.token,
                this.envService.tokenType,
                this.envService.userAgent,
                this.envService.workspaceId,
              ],
            },
          ],
        },
      },
    });
  }
}

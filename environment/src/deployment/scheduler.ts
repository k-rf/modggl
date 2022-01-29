import * as gcp from '@pulumi/gcp';
import * as pulumi from '@pulumi/pulumi';

import { Environment, ProductName, Region } from '../constant';

type Props = {
  productName: ProductName;
  environment: Environment;
  region: Region;
};

export class Scheduler {
  jobName: string;
  environment: Environment;
  region: Region;

  constructor(props: Props) {
    this.jobName = `${props.productName}-scheduler-job`;
    this.environment = props.environment;
    this.region = props.region;
  }

  create(topicName: pulumi.Input<string>): gcp.cloudscheduler.Job {
    return new gcp.cloudscheduler.Job(this.jobName, {
      name: this.jobName,
      description: 'The scheduler job for modggl',
      project: this.environment,
      pubsubTarget: {
        topicName,
        data: Buffer.from('Cloud Scheduler triggered').toString('base64'),
      },
      region: this.region,
      schedule: '0 9 * * *',
      timeZone: 'Asia/Tokyo',
    });
  }
}

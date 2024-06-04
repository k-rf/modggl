import * as gcp from '@pulumi/gcp';
import * as pulumi from '@pulumi/pulumi';

import { PRODUCTION_PROJECT, PRODUCT_NAME, REGION } from '../constant';

import { EnvService } from '../lib/env.service';
import { Scheduler } from './scheduler';

const config = new pulumi.Config();
const envService = new EnvService();

const productName = 'modggl';

const topic = new gcp.pubsub.Topic(productName, {
  name: productName,
});

const run = new pulumi.StackReference(
  `${config.require('org')}/modggl-pipeline/pipeline`
);

const subscriptionName = 'modggl-subscription';
const subscription = new gcp.pubsub.Subscription(subscriptionName, {
  topic: topic.id,
  name: subscriptionName,
  pushConfig: {
    pushEndpoint: run.getOutput('productionRunEndpoint'),
    oidcToken: {
      // TODO: 自動化する（環境変数にべた書きの値を取得している）
      serviceAccountEmail: envService.serviceAccountPushProduction.value,
    },
  },
  ackDeadlineSeconds: 60,
  retryPolicy: {
    minimumBackoff: '10s',
    maximumBackoff: '600s',
  },
  messageRetentionDuration: '600s',
});
export const result = subscription.name;

const schedulerBuilder = new Scheduler({
  environment: PRODUCTION_PROJECT,
  productName: PRODUCT_NAME,
  region: REGION,
});
const scheduler = schedulerBuilder.create(topic.id);
export const schedulerId = scheduler.id;

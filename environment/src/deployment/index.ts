import * as gcp from '@pulumi/gcp';

import { PRODUCTION_PROJECT, PRODUCT_NAME, REGION } from '../constant';

import { Scheduler } from './scheduler';

const productName = 'modggl';

const topic = new gcp.pubsub.Topic(productName, {
  name: productName,
});

const subscriptionName = 'integromat';
const subscription = new gcp.pubsub.Subscription(subscriptionName, {
  topic: topic.id,
  name: subscriptionName,
});
export const result = subscription.name;

const schedulerBuilder = new Scheduler({
  environment: PRODUCTION_PROJECT,
  productName: PRODUCT_NAME,
  region: REGION,
});
const scheduler = schedulerBuilder.create(topic.id);
export const schedulerId = scheduler.id;

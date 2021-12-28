import * as gcp from '@pulumi/gcp';

const productName = 'modggly-rs';

const topic = new gcp.pubsub.Topic(productName, {
  name: productName,
});

const subscriptionName = 'integromat';
const subscription = new gcp.pubsub.Subscription(subscriptionName, {
  topic: topic.id,
  name: subscriptionName,
});
export const result = subscription.name;

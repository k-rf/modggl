import * as gcp from '@pulumi/gcp';

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

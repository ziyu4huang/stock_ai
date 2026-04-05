import { FubonSDK } from 'fubon-neo';

const sdk = new FubonSDK();

const result = sdk.apikeyLogin(
  process.env.FUBON_PERSONAL_ID!,
  process.env.FUBON_API_KEY!,
  process.env.FUBON_CERT_PATH!,
  process.env.FUBON_CERT_PASS!
);

if (result.isSuccess) {
  console.log('Login OK:', result.data);
} else {
  console.error('Login failed:', result.message);
  process.exit(1);
}

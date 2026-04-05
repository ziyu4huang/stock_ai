import { FubonSDK, Mode } from 'fubon-neo';
import { resolve } from 'path';

let sdk: FubonSDK | null = null;

// Resolve cert path relative to project root (3 levels up from this file)
const PROJECT_ROOT = resolve(import.meta.dir, '..', '..', '..');

export async function getSDK(): Promise<FubonSDK> {
  if (sdk) return sdk;

  const certPath = process.env.FUBON_CERT_PATH!;
  const resolvedCertPath = resolve(PROJECT_ROOT, certPath);

  console.log(`FBS login: id=${process.env.FUBON_PERSONAL_ID}, cert=${resolvedCertPath}`);
  sdk = new FubonSDK();
  const result = sdk.apikeyLogin(
    process.env.FUBON_PERSONAL_ID!,
    process.env.FUBON_API_KEY!,
    resolvedCertPath,
    process.env.FUBON_CERT_PASS!
  );
  if (result.isSuccess) {
    console.log('FBS login OK:', result.data);
  } else {
    // Non-fatal warning (e.g. risk disclosure not signed yet) — SDK still works
    console.log(`FBS login warning: ${result.message}`);
  }
  sdk.initRealtime(Mode.Speed);
  console.log('FBS realtime initialized');
  return sdk;
}

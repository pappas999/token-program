import {
    establishConnection,
    establishPayer,
    checkProgram,
    createToken,
    createTokenAccounts,
    mint,
    transfer
  } from './token-program';

  async function main() {
    console.log("Let's create a token...");

    // Establish connection to the cluster
    await establishConnection();

    // Determine who pays for the fees
    await establishPayer();

    // Check if the program has been deployed
    await checkProgram();

    // Say hello to an account
    await createToken();

    // Say hello to an account
    await createTokenAccounts();

    // Say hello to an account
    await mint();

    // Say hello to an account
    await transfer();

    // Find out how many times that account has been greeted
    //await getTokenBalance();

    console.log('Success');
  }

  main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );

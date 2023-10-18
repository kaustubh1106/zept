import './App.css';
import { Contract, networks } from 'test1-client';
import { isAllowed, setAllowed, getUserInfo } from '@stellar/freighter-api';
function App() {
  const greeter = new Contract({
    ...networks.testnet,
    rpcUrl: 'https://soroban-testnet.stellar.org', // from https://soroban.stellar.org/docs/reference/rpc#public-rpc-providers
    });
  console.log("hello: ",greeter)
  // const wrap = document.querySelector('#freighter-wrap');
  // const ellipsis = document.querySelector('#freighter-wrap .ellipsis');
  // const button = document.querySelector('[data-connect]');

  const getPk = async()=> {
    const  publicKey  = await getUserInfo();
    console.log(publicKey)
    return publicKey;
  }
  getPk("hiii kaus")
  const connect = async ()=>{
    await setAllowed();
  }
//   const setLoggedIn = async () => {
//     const publicKey = await getPk();
//     ellipsis.innerHTML = `Signed in as ${publicKey}`;
//     ellipsis.title = publicKey;
//   }

//   if (await isAllowed()) {
//     if (await getPk()) setLoggedIn();
//     else wrap.innerHTML = 'Freighter is locked.<br>Sign in & refresh the page.';
//   } else {
//     button.addEventListener('click', async () => {
//       button.disabled = true;
//       await setAllowed();
//       await setLoggedIn();
//     });
//   }
// }
  return (
    <div className="App">
      <button onClick={connect}>connect</button><br/>
      <button onClick={getPk}>showpublickey</button>
    </div>
  );
}

export default App;

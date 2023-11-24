import axios from 'axios'
import './App.css';

function App() {
  

  const abc = async()=>{
    const response = await axios.post("https://logix-backend.onrender.com/setconditions",{
        shipment_id: 897,
        temperature_upper_limit: 20,
        temperature_lower_limit: 30,
        humidity_upper_limit: 10,
        humidity_lower_limit: 30
      })
      console.log(response)
    }
 
  return (
    <div className="App">
      <button onClick={abc}>get</button>
    </div>
  );
}

export default App;

import Menu from "./Menu";
import CodeMirror from '@uiw/react-codemirror';
import styled from "styled-components";
import init, { evaluate } from 'web_bindings';
import { useEffect, useState } from "react";

const Workspace = styled.div`
  width: 100%;
  max-width: 800px;
  margin-left:auto;
  margin-right:auto;
  background:rgba(0,0,0,0.05);
  box-sizing: border-box;
`;


function App() {
  const [loaded, setLoaded] = useState(false);
  const [source, setSource] = useState("");
  const [result, setResult] = useState("");

  // Load the WASM file
  useEffect(() => {
    init().then(() => setLoaded(true));
  }, []);

  useEffect(() => {
    if (!loaded) return;

    setResult(evaluate(source));
  }, [source])
  
  return <div className="App">
      <Menu />
      {loaded && <Workspace>
        <h1>Hello</h1>
        <CodeMirror
          onChange={setSource}
          value={source}
        />
        {result}
      </Workspace>}
    </div>
}

export default App

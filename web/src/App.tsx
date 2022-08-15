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
  box-sizing: border-box;
`;

const Result = styled.div`
  padding:1rem;
  box-sizing: border-box;
  background-color: rgba(0,0,0, 0.05);
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
        <CodeMirror
          onChange={setSource}
          value={source}
        />
        <Result>
          {result}
        </Result>
      </Workspace>}
    </div>
}

export default App

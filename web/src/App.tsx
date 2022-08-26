import Menu from "./Menu";
import { createTheme } from '@uiw/codemirror-themes';
import CodeMirror from '@uiw/react-codemirror';
import { tags as t } from '@lezer/highlight';
import styled from "styled-components";
import init, { read_cell, write_cell, insert_cell } from 'web_bindings';
import { MdAddCircleOutline, MdFormatAlignLeft } from "react-icons/md";
import { useEffect, useState } from "react";
import Convert from "ansi-to-html";

const theme = createTheme({
  theme: 'light',
  settings: {
    background: '#ffffff',
    foreground: '#1f1f1f',
    caret: '#AEAFAD',
    selection: '#D6D6D6',
    selectionMatch: '#D6D6D6',
    gutterBackground: '#FFFFFF',
    gutterForeground: '#4D4D4C',
    gutterBorder: '#ddd',
    lineHighlight: '#EFEFEF',
  },
  styles: [
    { tag: t.comment, color: '#787b80' },
    { tag: t.definition(t.typeName), color: '#194a7b' },
    { tag: t.typeName, color: '#194a7b' },
    { tag: t.tagName, color: '#008a02' },
    { tag: t.variableName, color: '#1a00db' },
  ],
});

const Workspace = styled.div`
  width: 100%;
  max-width: 800px;
  margin-left:auto;
  margin-right:auto;
  box-sizing: border-box;
  padding: 1rem;
`;

const Result = styled.div`
  padding:1rem;
  font-size:0.8rem;
  line-height: 1;
  box-sizing: border-box;
  background-color: rgba(0,0,0, 0.05);
  font-family: 'Roboto Mono', monospace;
  overflow-y: auto;
`;

const Actions = styled.div`
  display: flex;
  max-width: 300px;
  width: 100%;
`

const Action = styled.button`
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-sizing: border-box;
  padding: 0.5rem;
  width:100%;
  margin-top: 0.5rem;
  margin-right: 0.5rem;
`;

function App() {
  const [loaded, setLoaded] = useState(false);
  const [source, setSource] = useState("");
  const [result, setResult] = useState("");

  // Load the WASM file
  useEffect(() => {
    init().then(() => {
      setLoaded(true);
      insert_cell(0); // FIXME: get a more permanent solution for this
    });
  }, []);

  useEffect(() => {
    if (!loaded) return;
    write_cell(0, source);
    const output = read_cell(0);
    let converter = new Convert({newline: true}); 
    setResult(converter.toHtml(output));
  }, [source]);

  
  
  return <div className="App">
      <Menu />
      {loaded && <Workspace>
        <CodeMirror
          onChange={setSource}
          value={source}
          theme={theme}
          autoFocus
          basicSetup={{
            lineNumbers: true,
          }}
        />
        {
          source.trim() && 
            <pre><Result dangerouslySetInnerHTML={{__html: result}} /></pre>
        }
        <Actions>
          <Action>
            New Cell <MdAddCircleOutline size="1.2rem" />
          </Action>
          <Action>
            Text block <MdFormatAlignLeft size="1.2rem" />
          </Action>
        </Actions>

      </Workspace>}
    </div>
}

export default App

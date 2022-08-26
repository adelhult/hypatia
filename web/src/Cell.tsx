import React from "react";
import { createTheme } from '@uiw/codemirror-themes';
import CodeMirror from '@uiw/react-codemirror';
import { tags as t } from '@lezer/highlight';
import styled from "styled-components";
import Convert from "ansi-to-html"; 

const Result = styled.div`
  padding:1rem;
  font-size:0.8rem;
  line-height: 1;
  box-sizing: border-box;
  background-color: rgba(0,0,0, 0.05);
  font-family: 'Roboto Mono', monospace;
  overflow-y: auto;
  margin-bottom: 3rem;
`;

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


interface CellProps {
    code: string,
    output: string,
    index: number,
    onChange: (cell_index: number, code: string) => void,
}

function Cell(props: CellProps) {
    const converter = new Convert();

    return <>
        <CodeMirror
            onChange={code => props.onChange(props.index, code)}
            value={props.code}
            theme={theme}
            autoFocus
            basicSetup={{
                lineNumbers: true,
            }}
        />
        {
            <pre><Result dangerouslySetInnerHTML={{__html: converter.toHtml(props.output)}} /></pre>
        }
    </>
}

export default Cell;
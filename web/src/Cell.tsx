import React, { useState } from "react";
import { createTheme } from "@uiw/codemirror-themes";
import { keymap } from "@codemirror/view";
import CodeMirror from "@uiw/react-codemirror";
import { tags as t } from "@lezer/highlight";
import styled from "styled-components";
import { MdClose } from "react-icons/md";
import Convert from "ansi-to-html";
import { motion } from "framer-motion";

const Result = styled.div`
    box-sizing: border-box;
    font-family: 'JetBrains Mono', monospace;
    margin-top: 0.2rem;
`;

const ResultData = styled.div`
    font-size:1rem;
    line-height: 1;
    overflow-y: hidden;
    overflow-x: auto;
    
`;

const AnswerText = styled.span`
  opacity: 0.7;
  font-size: 0.8rem;  
`;

const Time = styled.span`
    font-size: 0.8rem;
    opacity: 0.7;
    position: absolute;
    right: 0.5rem;
    bottom: 0.5rem;
`;

const Wrapper = styled(motion.div)`
    position: relative;
    margin-bottom: 1.5rem;
`;

const Remove = styled.button`
    z-index: 1000;
    position: absolute;
    right: 3px;
    top: 6px;
    font-size: 1rem;
    background: none;
    border: none;
    opacity: 0.5;
    cursor: pointer;
    transition: opacity 0.2s;

    &:hover {
        opacity: 1;
    }
`;

const Formats = styled.div`
    display: flex;
    gap: 0.5rem;
    right: 0.5rem;
    position: absolute;
`;

const FormatButton = styled.button`
    border: solid;
    border-width: 1px;
    border-color: #d9d9d9;
    background: white;
    color: inherit;
    font-size: 0.8rem;
    padding: 0.2rem;
    border-radius: 0.2rem;
    transition: all 0.3s;
`;

const Editor = styled.div`
  border: solid 2px #D6D6D6;
`;

const hotkeys = (addCellAction: () => void) =>
  keymap.of([{
    key: "Alt-Enter",
    run() {
      addCellAction();
      return true;
    },
  }]);

const theme = createTheme({
  theme: "light",
  settings: {
    background: "#ffffff",
    foreground: "#1f1f1f",
    caret: "#AEAFAD",
    selection: "#D6D6D6",
    selectionMatch: "#D6D6D6",
    gutterBackground: "#FFFFFF",
    gutterForeground: "#4D4D4C",
    gutterBorder: "#ddd",
    lineHighlight: "#EFEFEF",
  },
  styles: [
    { tag: t.comment, color: "#787b80" },
    { tag: t.definition(t.typeName), color: "#194a7b" },
    { tag: t.typeName, color: "#194a7b" },
    { tag: t.tagName, color: "#008a02" },
    { tag: t.variableName, color: "#1a00db" },
  ],
});

interface CellProps {
  code: string;
  output: string;
  time?: string;
  index: number;
  noAnimation?: boolean;
  addCellAction: () => void;
  onRemove: (cell_index: number) => void;
  onChange: (cell_index: number, code: string) => void;
}

const Cell = React.memo((props: CellProps) => {
  const converter = new Convert();
  const [currentFormat, setCurrentFormat] = useState<null | string>(null);

  // Read the output.
  // First split the output into each format (seperated by %%%)
  // and then between the name and value which is seperated by ###
  const output = props.output
    .split("%%%")
    .map((format) => {
      let [value, name] = format.split("###");
      return [value, name];
    });

  const [outputValue, outputFormat] = output
    ?.find(([value, name]) => name === currentFormat) ?? output[0];

  return (
    <Wrapper
      initial={!props.noAnimation && { y: -20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
    >
      <Remove title="Remove cell" onClick={() => props.onRemove(props.index)}>
        <MdClose />
      </Remove>
      <Editor>
        <CodeMirror
          onChange={(code) => props.onChange(props.index, code)}
          value={props.code}
          theme={theme}
          autoFocus
          placeholder={props.index == 0 ? "try '10 m + 2 km'" : undefined}
          extensions={[hotkeys(props.addCellAction)]}
          basicSetup={{
            lineNumbers: props.code.match("\n") != null,
          }}
        />
      </Editor>

      {props.code && (
        <Result>
          <Formats>
            {output.map(([value, format]) =>
              format && (
                <FormatButton
                  style={{
                    opacity: format === outputFormat ? 1 : 0.3,
                  }}
                  key={format}
                  onClick={() => setCurrentFormat(format)}
                >
                  {format}
                </FormatButton>
              )
            )}
          </Formats>
          <AnswerText>answer({props.index}) =</AnswerText>
          <Time>{props.time ?? ""}</Time>
          <pre><ResultData dangerouslySetInnerHTML={{__html: converter.toHtml(outputValue)}}/>
          </pre>
        </Result>
      )}
    </Wrapper>
  );
});

export default Cell;

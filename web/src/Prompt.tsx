import styled from "styled-components";
import { MdClose } from "react-icons/md";
import { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";

const Box = styled(motion.div)`
    position: absolute;
    width: 100%;
    top: 0;
    display: flex;
    gap: 2rem;
    align-items: flex-start;
    justify-content: center;
    background: #4C956C;
    color: white;
    padding: 1rem;
    box-sizing: border-box;

    @media (max-width: 800px) {
        flex-direction: column;
        gap: 0.5rem;
    }
`;

const Remove = styled.button`
    z-index: 1000;
    position: absolute;
    right: 1rem;
    font-size: 1rem;
    background: none;
    border: none;
    opacity: 1;
    cursor: pointer;
    transition: opacity 0.2s;

    &:hover {
        opacity: 1;
    }
`;

const Action = styled.button`
    background: none;
    border: none;
    text-decoration: underline;
    color: inherit;
    font-weight: normal;
    font-family: inherit;
    padding:0;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
        background:rgba(1,1,1, 0.1);
    }
`;

interface PromptProps {
  children: React.ReactNode;
  action: string;
  title?: string;
  show: boolean;
  handleAction: () => void;
}

export default function Prompt(props: PromptProps) {
  const [show, setShow] = useState(true);

  return (
    <AnimatePresence>
      {show && props.show && (
        <Box
          initial={{ translateY: "-150%" }}
          animate={{ translateY: "0%" }}
          exit={{ translateY: "-150%" }}
          transition={{ easings: ["easeIn", "easeOut"] }}
          className="Prompt"
        >
          <Remove
            title="Close prompt"
            onClick={() => setShow(false)}
          >
            <MdClose style={{ color: "white" }} />
          </Remove>
          <div>
            {props.title && <strong>{props.title}</strong>} {props.children}
          </div>
          <Action onClick={props.handleAction}>
            {props.action}
          </Action>
        </Box>
      )}
    </AnimatePresence>
  );
}

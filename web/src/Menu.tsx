import styled from "styled-components";
import { FaBook, FaGithub } from "react-icons/fa";
import Button from "./Button";

const Container = styled.div`
    position: sticky;
    top: 0;
    left: 0;
    z-index: 100;
    width:100%;
`;

const Prompts = styled.div``;

const Buttons = styled.div`
  position: absolute;
  width: 100%;
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding: 1rem;
  box-sizing: border-box;
`;

type MenuProps = {
  toggleHelp: () => void;
  helpOpen: boolean;
  prompts: JSX.Element[]
};

export default function Menu({ toggleHelp, helpOpen, prompts }: MenuProps) {
  return (
    <Container className="Menu">
      <Prompts>
        {prompts}
      </Prompts>
      <Buttons>
        <Button
          onClick={() => location.href = "https://github.com/adelhult/hypatia"}
          title="Github"
          icon={<FaGithub />}
        />
        <Button
          onClick={toggleHelp}
          active={helpOpen}
          title="Help"
          icon={<FaBook />}
        />
      </Buttons>
    </Container>
  );
}

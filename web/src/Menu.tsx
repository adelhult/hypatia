import styled from "styled-components";
import { FaBook, FaGithub } from "react-icons/fa";
const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: space-between;
    align-items: center;  
`;

const LogoContainer = styled.div`
  display: flex;
  align-items: center;  
`;

const Logo = styled.img`
    width: 6rem;
    margin-right: 0.5rem;
`;

const LogoText = styled.h1`
    font-size: 1.6rem;
    font-weight: normal;
`;

const Buttons = styled.div`
  display: flex;
  gap: 1rem;

  &>button {
  border: solid 2px black;
  border-radius: 0.2rem;
  padding: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.9rem;
}
`;

type MenuProps = {
  toggleHelp: () => void;
};

export default function Menu({ toggleHelp }: MenuProps) {
  return (
    <Container className="Menu">
      <LogoContainer>
        <Logo src="logo.png"></Logo>
        <LogoText>
          <strong>Hypatia</strong>
          <br />Calculator
        </LogoText>
      </LogoContainer>
      <Buttons>
        <button
          onClick={() => location.href = "https://github.com/adelhult/hypatia"}
        >
          Github <FaGithub />
        </button>
        <button onClick={toggleHelp}>
          Help <FaBook />
        </button>
      </Buttons>
    </Container>
  );
}

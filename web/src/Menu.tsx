import styled from 'styled-components';
const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
`

const LogoContainer = styled.div`
  display: flex;
  align-items: center;  
`;

const Logo = styled.img`
    max-width: 8rem;
    margin-right: 0.5rem;
`

const LogoText = styled.h1`
    font-size: 1.6rem;
    font-weight: normal;
`

export default function Menu() {
    return <Container className="Menu">
        <LogoContainer>
            <Logo src="logo.png"></Logo>
            <LogoText><strong>Hypatia</strong><br/> Notebook</LogoText>
        </LogoContainer>
    </Container>
}
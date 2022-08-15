import styled from 'styled-components';


const Button = styled.button`
    color: red;
`;

const Container = styled.div`
    display: flex;
    width:100%;
    box-sizing: border-box;
    padding:1rem;
`

const Logo = styled.div`
    font-size: 2rem;
`

export default function Menu() {
    return <Container className="Menu">
        <Logo>Hypatia</Logo>
        <Button>Test</Button>
    </Container>
}
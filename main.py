import streamlit as st
from sympy import symbols, Eq
x, y, z = symbols('x y z')

st.write("# Resuelve tu ecuación")

func = st.text_input("Introduce tu función")
metodo = st.selectbox('Selecciona un método', ["Bisección", "NEWTON-RAPHSON", "SECANTE"])

if st.button("Arre!"):
    if metodo == "Bisección":
        st.latex(r''' m = \frac{a+b}{2} ''')
    elif metodo == "NEWTON-RAPHSON":
        st.latex(r''' X_{n+1} = X_{n} - \frac{f(X_n)}{f'(X_n)}''')
    elif metodo == "SECANTE":
        st.latex(r''' X_{n+1} = X_{n} - \frac{X_n-X_{n-1}}{f(X_n)-f(X_{n-1})}f(X_n)''')

    st.write(metodo)
with st.sidebar:
    st.write("# Equipo 1")
    st.write("Daniel Osornio")
    st.write("Daniel Hernández")
    st.write("Lorenzo Reinoso")
    st.write("Iván Dominguez")
    st.write("Abril Bautista")

st.latex(r''' e^{i\pi} + 1 = 0 ''')

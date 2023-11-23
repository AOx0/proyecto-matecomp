import streamlit as st
from sympy import symbols, Eq
import matplotlib.pyplot as plt
import numpy as np

x = symbols('x')

st.write("# Resuelve tu ecuación")

func = st.text_input("Introduce tu ecuación")
metodo = st.selectbox('Selecciona un método', ["Bisección", "NEWTON-RAPHSON", "SECANTE"])
bar1, bar2 = st.columns([1, 1])

interations = bar1.slider('error = 10x10^', min_value=1, max_value=10)

car1, car2, car3 = bar2.columns([1, 1, 1])
if car2.button("resuelve!"):
    col1, col2 = st.columns([1, 1])
    x_values = np.linspace(-10, 10, 100)


    def evaluate_expression(expression, x_values):
        try:
            y_values = [eval(expression) for x in x_values]
            return y_values
        except Exception as e:
            st.error(f"Error al evaluar la expresión: {e}")
            return None


    y_values = evaluate_expression(func, x_values)
    if y_values is not None:
        fig, ax = plt.subplots()
        ax.plot(x_values, y_values, color='red')
        ax.set_title("Gráfica de la Función",color='white')
        ax.set_xlabel("x",color='white')
        ax.set_ylabel("y",color='white')
        ax.tick_params(axis='both', colors='white')
        fig.patch.set_facecolor('#0F1116')
        col2.pyplot(fig)

    if metodo == "Bisección":
        col1.write("Formula")
        col1.latex(r''' m = \frac{a+b}{2} ''')
        # se llama a la función y esta error y el valor o el procedimiento
        #
        # llamar a las funciones de RUST,
            # datos de entrada(función (derivada si aplica), y el error)
            # datos de salida (el numero de iteraciones y el valor aproximado)

    elif metodo == "NEWTON-RAPHSON":
        col1.write("Formula")
        col1.latex(r''' X_{n+1} = X_{n} - \frac{f(X_n)}{f'(X_n)}''')
    elif metodo == "SECANTE":
        col1.write("Formula")
        col1.latex(r''' X_{n+1} = X_{n} - \frac{X_n-X_{n-1}}{f(X_n)-f(X_{n-1})}f(X_n)''')

    # LLAMAR A LAS FUNCIONES
    y = eval(func)
    yprima = y.diff(x)
    col1.write("Función")
    col1.write(y)
    col1.write(yprima)
    col1.write(metodo)

on = st.toggle('Mostrar codigo')
if on:
    st.code('for i in range(8): foo()')

with st.sidebar:
    st.write("# Equipo 1")
    st.write("Daniel Osornio")
    st.write("Daniel Hernández")
    st.write("Lorenzo Reinoso")
    st.write("Iván Dominguez")
    st.write("Abril Bautista")

st.latex(r''' e^{i\pi} + 1 = 0 ''')

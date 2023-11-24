import sys
import streamlit as st
from sympy import symbols, Eq, diff, evalf
import matplotlib.pyplot as plt
import numpy as np
import random

x = symbols('x')


# ESPACIO DE LAS FUNCIONES

def secante(funcion, error, value):
    funcion = eval(funcion)
    # 40, 39
    solp1 = 10
    # solm1 = random.uniform(1, 10)
    solm1 = 11
    iterations = 0
    mistake = 1

    while mistake >= error:
        iterations += 1
        a = solp1
        solp1 = solp1 - \
                (
                        (solp1 - solm1) /
                        (funcion.evalf(subs={x: solp1}) - funcion.evalf(subs={x: solm1}))
                        * (funcion.evalf(subs={x: solp1}))
                )
        solm1 = a
        mistake = abs(solp1 - solm1) / 2.
        print(f'Iteración {iterations}: {solp1}')

    return iterations, solp1


def bisection_method(funcion, error):
    funcion = eval(funcion)
    a = -100.0
    b = 100.0
    iterations = 0
    y_value = 0
    while (b - a) / 2 >= error:
        c = (b + a) / 2.0
        y_value = c
        if funcion.evalf(subs={x: c}) >= 0:
            b = c
        else:
            a = c
        iterations += 1
    return iterations, y_value


def calc_error(punto_n, punto_v):
    e = abs((punto_n - punto_v) / punto_n)
    return e


def newton_raphson(funcion, error_g):
    g_func = eval(funcion)
    g_func_prima = g_func.diff(x)
    error = 1.0
    iteracion = 0
    X_val = random.uniform(1, 100)
    X_new = 0
    while error >= error_g:
        X_new = X_val - ((g_func.evalf(subs={x: X_val})) / (g_func_prima.evalf(subs={x: X_val})))
        error = calc_error(X_new, X_val)
        iteracion += 1
        print(f'Iteración {iteracion}: {X_new}')
        X_val = X_new
    return iteracion, X_new


#


st.write("# Resuelve tu ecuación")

func = st.text_input("Introduce tu ecuación")
metodo = st.selectbox('Selecciona un método', ["Bisección", "NEWTON-RAPHSON", "SECANTE"])
bar1, bar2 = st.columns([1, 1])

mistake = bar1.slider('error = 10x10^-', min_value=1, max_value=10)

car1, car2, car3 = bar2.columns([1, 1, 1])
if car2.button("resuelve!"):
    col1, col2 = st.columns([1, 1])
    x_values = np.linspace(-10, 10, 100)
    iteraciones = 0
    root = 0


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
        ax.set_title("Gráfica de la Función", color='white')
        ax.set_xlabel("x", color='white')
        ax.set_ylabel("y", color='white')
        ax.tick_params(axis='both', colors='white')
        fig.patch.set_facecolor('#0F1116')
        col2.pyplot(fig)

    if metodo == "Bisección":
        col1.write("Formula")
        col1.latex(r''' m = \frac{a+b}{2} ''')
        iteraciones, root = bisection_method(func, 10 * 10 ** (-mistake))


    elif metodo == "NEWTON-RAPHSON":
        col1.write("Formula")
        col1.latex(r''' X_{n+1} = X_{n} - \frac{f(X_n)}{f'(X_n)}''')
        iteraciones, root = newton_raphson(func, 10 * 10 ** (-mistake))

    elif metodo == "SECANTE":
        col1.write("Formula")
        col1.latex(r''' X_{n+1} = X_{n} - \frac{X_n-X_{n-1}}{f(X_n)-f(X_{n-1})}f(X_n)''')

    # LLAMAR A LAS FUNCIONES
    y = eval(func)
    col1.write("Función")
    col1.write(y)
    st.success(f'''
                    Numero de Iteraciones:{iteraciones}
                    
                    Raiz: {root}
                    
                    valor aproximado: {(y.evalf(subs={x: root}))}
                ''', icon="✅")

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

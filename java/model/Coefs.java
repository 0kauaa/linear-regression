package model;

import java.util.ArrayList;
import java.util.List;

public class Coefs {

    // soma de lista
    private static double sum(List<Double> vec) {
        double res = 0;
        
        for (int i = 0; i < vec.size(); i++) {
            res += vec.get(i);
        }

        return res;
    }

    // multiplicação de listas
    private static List<Double> mult(List<Double> vec1, List<Double> vec2) {
        List<Double> res = new ArrayList<>();

        for (int i = 0; i < vec1.size() && i < vec2.size(); i++) {
            res.add(vec1.get(i) * vec2.get(i));
        }

        return res;
    }

    // coeficiente angular
    public static double b1(List<Double> x, List<Double> y) {
        int n      = x.size();
        double xs  = sum(x);
        double ys  = sum(y);
        double xy  = sum(mult(x, y));
        double x2 = x.stream().mapToDouble(val -> val * val).sum();

        double num = n * xy - xs * ys;
        double div = n * x2 - (xs*xs);
        
        return num / div;
    }

    // intercepto
    public static double b0(List<Double> x, List<Double> y) {
        int n     = x.size();
        double ys = sum(y);
        double xs = sum(x);

        return  (ys - b1(x, y) * xs) / n;
    }
}
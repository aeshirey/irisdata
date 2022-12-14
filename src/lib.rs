#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Species {
    /// [Iris setosa](https://en.wikipedia.org/wiki/Iris_setosa), the bristle-pointed iris.
    IrisSetosa,

    /// [Iris versicolor](https://en.wikipedia.org/wiki/Iris_versicolor), commonly known as the
    /// blue flag.
    IrisVersicolor,

    /// [Iris virginica](https://en.wikipedia.org/wiki/Iris_virginica), commonly known as the
    /// Virginia blueflag.
    IrisVirginica,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Iris {
    /// Length of the sepal in centimeters
    pub sepal_length: f32,

    /// Width of the sepal in centimeters
    pub sepal_width: f32,

    /// Length of the petal in centimeters
    pub petal_length: f32,

    /// Width of the petal in centimeters
    pub petal_width: f32,

    /// Identified [Species] of iris
    pub species: Species,
}

/// [Fisher's Iris data set](https://en.wikipedia.org/wiki/Iris_flower_data_set).
pub const IRIS_DATA: &[Iris; 150] = &[
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.5,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 3.0,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.7,
        sepal_width: 3.2,
        petal_length: 1.3,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.6,
        sepal_width: 3.1,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.6,
        petal_length: 1.4,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.9,
        petal_length: 1.7,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.6,
        sepal_width: 3.4,
        petal_length: 1.4,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.4,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.4,
        sepal_width: 2.9,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 3.1,
        petal_length: 1.5,
        petal_width: 0.1,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.7,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.8,
        sepal_width: 3.4,
        petal_length: 1.6,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.8,
        sepal_width: 3.0,
        petal_length: 1.4,
        petal_width: 0.1,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.3,
        sepal_width: 3.0,
        petal_length: 1.1,
        petal_width: 0.1,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 4.0,
        petal_length: 1.2,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 4.4,
        petal_length: 1.5,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.9,
        petal_length: 1.3,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.5,
        petal_length: 1.4,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 3.8,
        petal_length: 1.7,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.8,
        petal_length: 1.5,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.4,
        petal_length: 1.7,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.7,
        petal_length: 1.5,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.6,
        sepal_width: 3.6,
        petal_length: 1.0,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.3,
        petal_length: 1.7,
        petal_width: 0.5,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.8,
        sepal_width: 3.4,
        petal_length: 1.9,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.0,
        petal_length: 1.6,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.4,
        petal_length: 1.6,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.2,
        sepal_width: 3.5,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.2,
        sepal_width: 3.4,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.7,
        sepal_width: 3.2,
        petal_length: 1.6,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.8,
        sepal_width: 3.1,
        petal_length: 1.6,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.4,
        petal_length: 1.5,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.2,
        sepal_width: 4.1,
        petal_length: 1.5,
        petal_width: 0.1,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 4.2,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 3.1,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.2,
        petal_length: 1.2,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 3.5,
        petal_length: 1.3,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 3.6,
        petal_length: 1.4,
        petal_width: 0.1,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.4,
        sepal_width: 3.0,
        petal_length: 1.3,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.4,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.5,
        petal_length: 1.3,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.5,
        sepal_width: 2.3,
        petal_length: 1.3,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.4,
        sepal_width: 3.2,
        petal_length: 1.3,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.5,
        petal_length: 1.6,
        petal_width: 0.6,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.8,
        petal_length: 1.9,
        petal_width: 0.4,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.8,
        sepal_width: 3.0,
        petal_length: 1.4,
        petal_width: 0.3,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 3.8,
        petal_length: 1.6,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 4.6,
        sepal_width: 3.2,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.3,
        sepal_width: 3.7,
        petal_length: 1.5,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 3.3,
        petal_length: 1.4,
        petal_width: 0.2,
        species: Species::IrisSetosa,
    },
    Iris {
        sepal_length: 7.0,
        sepal_width: 3.2,
        petal_length: 4.7,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 3.2,
        petal_length: 4.5,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.9,
        sepal_width: 3.1,
        petal_length: 4.9,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 2.3,
        petal_length: 4.0,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.5,
        sepal_width: 2.8,
        petal_length: 4.6,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 2.8,
        petal_length: 4.5,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 3.3,
        petal_length: 4.7,
        petal_width: 1.6,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 2.4,
        petal_length: 3.3,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.6,
        sepal_width: 2.9,
        petal_length: 4.6,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.2,
        sepal_width: 2.7,
        petal_length: 3.9,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 2.0,
        petal_length: 3.5,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.9,
        sepal_width: 3.0,
        petal_length: 4.2,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 2.2,
        petal_length: 4.0,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 2.9,
        petal_length: 4.7,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 2.9,
        petal_length: 3.6,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.1,
        petal_length: 4.4,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 3.0,
        petal_length: 4.5,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.7,
        petal_length: 4.1,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.2,
        sepal_width: 2.2,
        petal_length: 4.5,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 2.5,
        petal_length: 3.9,
        petal_width: 1.1,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.9,
        sepal_width: 3.2,
        petal_length: 4.8,
        petal_width: 1.8,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 2.8,
        petal_length: 4.0,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.5,
        petal_length: 4.9,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 2.8,
        petal_length: 4.7,
        petal_width: 1.2,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 2.9,
        petal_length: 4.3,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.6,
        sepal_width: 3.0,
        petal_length: 4.4,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.8,
        sepal_width: 2.8,
        petal_length: 4.8,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.0,
        petal_length: 5.0,
        petal_width: 1.7,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 2.9,
        petal_length: 4.5,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 2.6,
        petal_length: 3.5,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 2.4,
        petal_length: 3.8,
        petal_width: 1.1,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 2.4,
        petal_length: 3.7,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.7,
        petal_length: 3.9,
        petal_width: 1.2,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 2.7,
        petal_length: 5.1,
        petal_width: 1.6,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.4,
        sepal_width: 3.0,
        petal_length: 4.5,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 3.4,
        petal_length: 4.5,
        petal_width: 1.6,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.1,
        petal_length: 4.7,
        petal_width: 1.5,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.3,
        petal_length: 4.4,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 3.0,
        petal_length: 4.1,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 2.5,
        petal_length: 4.0,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.5,
        sepal_width: 2.6,
        petal_length: 4.4,
        petal_width: 1.2,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 3.0,
        petal_length: 4.6,
        petal_width: 1.4,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.6,
        petal_length: 4.0,
        petal_width: 1.2,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.0,
        sepal_width: 2.3,
        petal_length: 3.3,
        petal_width: 1.0,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 2.7,
        petal_length: 4.2,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 3.0,
        petal_length: 4.2,
        petal_width: 1.2,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 2.9,
        petal_length: 4.2,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.2,
        sepal_width: 2.9,
        petal_length: 4.3,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.1,
        sepal_width: 2.5,
        petal_length: 3.0,
        petal_width: 1.1,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 2.8,
        petal_length: 4.1,
        petal_width: 1.3,
        species: Species::IrisVersicolor,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 3.3,
        petal_length: 6.0,
        petal_width: 2.5,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.7,
        petal_length: 5.1,
        petal_width: 1.9,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.1,
        sepal_width: 3.0,
        petal_length: 5.9,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.9,
        petal_length: 5.6,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.5,
        sepal_width: 3.0,
        petal_length: 5.8,
        petal_width: 2.2,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.6,
        sepal_width: 3.0,
        petal_length: 6.6,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 4.9,
        sepal_width: 2.5,
        petal_length: 4.5,
        petal_width: 1.7,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.3,
        sepal_width: 2.9,
        petal_length: 6.3,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 2.5,
        petal_length: 5.8,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.2,
        sepal_width: 3.6,
        petal_length: 6.1,
        petal_width: 2.5,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.5,
        sepal_width: 3.2,
        petal_length: 5.1,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 2.7,
        petal_length: 5.3,
        petal_width: 1.9,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.8,
        sepal_width: 3.0,
        petal_length: 5.5,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.7,
        sepal_width: 2.5,
        petal_length: 5.0,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.8,
        petal_length: 5.1,
        petal_width: 2.4,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 3.2,
        petal_length: 5.3,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.5,
        sepal_width: 3.0,
        petal_length: 5.5,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.7,
        sepal_width: 3.8,
        petal_length: 6.7,
        petal_width: 2.2,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.7,
        sepal_width: 2.6,
        petal_length: 6.9,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 2.2,
        petal_length: 5.0,
        petal_width: 1.5,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.9,
        sepal_width: 3.2,
        petal_length: 5.7,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.6,
        sepal_width: 2.8,
        petal_length: 4.9,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.7,
        sepal_width: 2.8,
        petal_length: 6.7,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.7,
        petal_length: 4.9,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.3,
        petal_length: 5.7,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.2,
        sepal_width: 3.2,
        petal_length: 6.0,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.2,
        sepal_width: 2.8,
        petal_length: 4.8,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 3.0,
        petal_length: 4.9,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 2.8,
        petal_length: 5.6,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.2,
        sepal_width: 3.0,
        petal_length: 5.8,
        petal_width: 1.6,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.4,
        sepal_width: 2.8,
        petal_length: 6.1,
        petal_width: 1.9,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.9,
        sepal_width: 3.8,
        petal_length: 6.4,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 2.8,
        petal_length: 5.6,
        petal_width: 2.2,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.8,
        petal_length: 5.1,
        petal_width: 1.5,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.1,
        sepal_width: 2.6,
        petal_length: 5.6,
        petal_width: 1.4,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 7.7,
        sepal_width: 3.0,
        petal_length: 6.1,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 3.4,
        petal_length: 5.6,
        petal_width: 2.4,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.4,
        sepal_width: 3.1,
        petal_length: 5.5,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.0,
        sepal_width: 3.0,
        petal_length: 4.8,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.9,
        sepal_width: 3.1,
        petal_length: 5.4,
        petal_width: 2.1,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.1,
        petal_length: 5.6,
        petal_width: 2.4,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.9,
        sepal_width: 3.1,
        petal_length: 5.1,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.8,
        sepal_width: 2.7,
        petal_length: 5.1,
        petal_width: 1.9,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.8,
        sepal_width: 3.2,
        petal_length: 5.9,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.3,
        petal_length: 5.7,
        petal_width: 2.5,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.7,
        sepal_width: 3.0,
        petal_length: 5.2,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.3,
        sepal_width: 2.5,
        petal_length: 5.0,
        petal_width: 1.9,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.5,
        sepal_width: 3.0,
        petal_length: 5.2,
        petal_width: 2.0,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 6.2,
        sepal_width: 3.4,
        petal_length: 5.4,
        petal_width: 2.3,
        species: Species::IrisVirginica,
    },
    Iris {
        sepal_length: 5.9,
        sepal_width: 3.0,
        petal_length: 5.1,
        petal_width: 1.8,
        species: Species::IrisVirginica,
    },
];

#[test]
fn count_species() {
    let mut setosa = 0;
    let mut versicolor = 0;
    let mut virginica = 0;

    for i in IRIS_DATA {
        match i.species {
            Species::IrisSetosa => setosa += 1,
            Species::IrisVersicolor => versicolor += 1,
            Species::IrisVirginica => virginica += 1,
        }
    }

    let avg = IRIS_DATA
        .iter()
        .filter_map(|i| {
            if i.species == Species::IrisVirginica {
                Some(i.petal_length)
            } else {
                None
            }
        })
        .sum::<f32>()
        / 50.;
    assert_eq!(5.5520005, avg);

    assert_eq!(setosa, 50);
    assert_eq!(versicolor, 50);
    assert_eq!(virginica, 50);
}

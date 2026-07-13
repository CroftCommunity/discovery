plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.croftc.p2pexp"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.croftc.p2pexp"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        // The Rust core only ships .so for these ABIs (see jniLibs / cargo-ndk).
        ndk {
            abiFilters += listOf("arm64-v8a", "x86_64")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    // UniFFI's generated Kotlin bindings use JNA to call into the native library.
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.8.1")
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
}

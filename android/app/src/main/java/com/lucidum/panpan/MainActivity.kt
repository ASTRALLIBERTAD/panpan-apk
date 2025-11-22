package com.lucidum.panpan

import android.app.Activity
import android.opengl.GLSurfaceView
import android.os.Bundle

class MainActivity : Activity() {

    companion object {
        init {
            System.loadLibrary("panpan")
        }
    }

    external fun nativeInit()
    external fun nativeResize(width: Int, height: Int)
    external fun nativeRender()

    private lateinit var glView: GLSurfaceView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        glView = object : GLSurfaceView(this) {
            override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
                super.onMeasure(widthMeasureSpec, heightMeasureSpec)
            }
        }

        glView.setEGLContextClientVersion(3)

        glView.setRenderer(object : GLSurfaceView.Renderer {
            override fun onSurfaceCreated(unused: javax.microedition.khronos.opengles.GL10?, config: javax.microedition.khronos.egl.EGLConfig?) {
                nativeInit()
            }

            override fun onSurfaceChanged(unused: javax.microedition.khronos.opengles.GL10?, width: Int, height: Int) {
                nativeResize(width, height)
            }

            override fun onDrawFrame(unused: javax.microedition.khronos.opengles.GL10?) {
                nativeRender()
            }
        })

        setContentView(glView)
    }
}

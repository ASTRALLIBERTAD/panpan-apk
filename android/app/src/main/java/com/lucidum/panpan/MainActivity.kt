package com.lucidum.panpan

import android.app.Activity
import android.opengl.GLSurfaceView
import android.os.Bundle
import android.view.MotionEvent

class MainActivity : Activity() {

    companion object {
        init {
            System.loadLibrary("panpan")
        }
    }

    external fun nativeInit()
    external fun nativeResize(width: Int, height: Int)
    external fun nativeRender()
    external fun nativeTouchDown(id: Int, x: Float, y: Float)
    external fun nativeTouchMove(id: Int, x: Float, y: Float)
    external fun nativeTouchUp(id: Int)
    external fun nativeUpdateTime(deltaTime: Float)

    private lateinit var glView: GLSurfaceView
    private var lastFrameTime = System.nanoTime()

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
                lastFrameTime = System.nanoTime()
            }

            override fun onSurfaceChanged(unused: javax.microedition.khronos.opengles.GL10?, width: Int, height: Int) {
                nativeResize(width, height)
            }

            override fun onDrawFrame(unused: javax.microedition.khronos.opengles.GL10?) {
                val currentTime = System.nanoTime()
                val deltaTime = (currentTime - lastFrameTime) / 1_000_000_000.0f
                lastFrameTime = currentTime
                
                nativeUpdateTime(deltaTime)
                nativeRender()
            }
        })

        setContentView(glView)
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        val action = event.actionMasked
        val pointerIndex = event.actionIndex
        val pointerId = event.getPointerId(pointerIndex)

        when (action) {
            MotionEvent.ACTION_DOWN, MotionEvent.ACTION_POINTER_DOWN -> {
                val x = event.getX(pointerIndex)
                val y = event.getY(pointerIndex)
                nativeTouchDown(pointerId, x, y)
            }
            MotionEvent.ACTION_MOVE -> {
                for (i in 0 until event.pointerCount) {
                    val id = event.getPointerId(i)
                    val x = event.getX(i)
                    val y = event.getY(i)
                    nativeTouchMove(id, x, y)
                }
            }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_POINTER_UP -> {
                nativeTouchUp(pointerId)
            }
            MotionEvent.ACTION_CANCEL -> {
                for (i in 0 until event.pointerCount) {
                    nativeTouchUp(event.getPointerId(i))
                }
            }
        }
        return true
    }

    override fun onPause() {
        super.onPause()
        glView.onPause()
    }

    override fun onResume() {
        super.onResume()
        glView.onResume()
    }
}

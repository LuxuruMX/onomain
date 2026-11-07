using System.Runtime.InteropServices;
using System.Text.Json;

const uint EstiloSuave = 2;
string[] semillas = ["bosque", "luna"];
IntPtr[] entradas = semillas.Select(Marshal.StringToCoTaskMemUTF8).ToArray();
try
{
    IntPtr resultado = Onomain.GenerarVarias(entradas, (nuint)entradas.Length, 12, EstiloSuave);
    if (resultado == IntPtr.Zero)
        throw new InvalidOperationException($"Onomain devolvió error {Onomain.UltimoError()}");

    try
    {
        string json = Marshal.PtrToStringUTF8(resultado)!;
        string[] palabras = JsonSerializer.Deserialize<string[]>(json)!;
        Console.WriteLine(string.Join(", ", palabras));
    }
    finally
    {
        Onomain.Liberar(resultado);
    }
}
finally
{
    foreach (IntPtr entrada in entradas)
        Marshal.FreeCoTaskMem(entrada);
}

internal static class Onomain
{
    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_generar_varias")]
    internal static extern IntPtr GenerarVarias([In] IntPtr[] semillas, nuint numeroSemillas,
        uint cantidad, uint estilo);

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_ultimo_error")]
    internal static extern uint UltimoError();

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_liberar")]
    internal static extern void Liberar(IntPtr texto);
}

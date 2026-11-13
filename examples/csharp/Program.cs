using System.Runtime.InteropServices;
using System.Text.Json;

const ulong Semilla = 12345;
string perfil = LeerYLiberar(Onomain.DescribirNumero(Semilla));
string[] palabras = JsonSerializer.Deserialize<string[]>(
    LeerYLiberar(Onomain.GenerarNumero(Semilla, 12)))!;
Console.WriteLine($"Semilla {Semilla}: {perfil}");
Console.WriteLine(string.Join(", ", palabras));

using JsonDocument detalles = JsonDocument.Parse(
    LeerYLiberar(Onomain.GenerarNumeroDetalle(Semilla, 3)));
foreach (JsonElement palabra in detalles.RootElement.EnumerateArray())
{
    string[] silabas = palabra.GetProperty("silabas").EnumerateArray()
        .Select(s => s.GetString()!).ToArray();
    int tonica = palabra.GetProperty("tonica").GetInt32();
    silabas[tonica] = silabas[tonica].ToUpperInvariant();
    Console.WriteLine($"{palabra.GetProperty("texto").GetString()}: {string.Join("-", silabas)}");
}

static string LeerYLiberar(IntPtr puntero)
{
    if (puntero == IntPtr.Zero)
        throw new InvalidOperationException($"Onomain devolvió error {Onomain.UltimoError()}");
    try { return Marshal.PtrToStringUTF8(puntero)!; }
    finally { Onomain.Liberar(puntero); }
}

internal static class Onomain
{
    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_generar_numero")]
    internal static extern IntPtr GenerarNumero(ulong semilla, uint cantidad);

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_generar_numero_detalle")]
    internal static extern IntPtr GenerarNumeroDetalle(ulong semilla, uint cantidad);

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_describir_numero")]
    internal static extern IntPtr DescribirNumero(ulong semilla);

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_ultimo_error")]
    internal static extern uint UltimoError();

    [DllImport("onomain", CallingConvention = CallingConvention.Cdecl, EntryPoint = "onomain_liberar")]
    internal static extern void Liberar(IntPtr texto);
}

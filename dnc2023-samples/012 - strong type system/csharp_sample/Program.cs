namespace CsharpSample;

public class User
{
    public string Name { get; set; }
    public bool IsActive { get; set; } = false;
    public DateTime? ActivationDate { get; set; }

    public void Activate(DateTime activationDate)
    {
        IsActive = true;
        ActivationDate = activationDate;
    }

    public bool SaveToDb()
    {
        return true;
    }
}
public class Program
{
    static void Main()
    {
        var mario = new User { Name = "Can Cey Maryo", IsActive = true, ActivationDate = DateTime.Now.AddYears(-1) };
        // mario önceki satırlarda bir yerlde oluşturulurken aktifleştirildi.
        // Ama zaten aktif idi. Tekrar aktifleştirecek bir metot çağırabiliriz.        
        // Peki IsActive False olduğunda aktivasyon tarihi null'a mı çekilmelidir?
        // Bunların önüne geçmek elbette mümkün hem de bir çok yolla. 
        // Birde benzer senaryoyu Rust ile nasıl yazacağımıza bakalım.
        
        mario.Activate(DateTime.Now);
    }
}
